#include <sys/syscall.h>
#include <sys/mman.h>
#include <linux/io_uring.h>
#include <unistd.h>
#include <string.h>
#include <stdio.h>

int main() {

	struct io_uring_params params;
	memset(&params, 0, sizeof(params));


	int file_desc = syscall(__NR_io_uring_setup, 8, &params);
	if (file_desc < 0) return 1;


	size_t sq_ring_sz =
		params.sq_off.array +
		params.sq_entries * sizeof(__u32);

	size_t cq_ring_sz =
		params.cq_off.cqes +
		params.cq_entries * sizeof(struct io_uring_cqe);



	void* sq_ring = mmap(
		NULL,
		sq_ring_sz,
		PROT_READ | PROT_WRITE,
		MAP_SHARED | MAP_POPULATE,
		file_desc,
		IORING_OFF_SQ_RING
	);
	if (sq_ring == -1) return 1;


	void* cq_ring = mmap(
		NULL,
		cq_ring_sz,
		PROT_READ | PROT_WRITE,
		MAP_SHARED | MAP_POPULATE,
		file_desc,
		IORING_OFF_CQ_RING
	);
	if (cq_ring == -1) return 1;


	struct io_uring_sqe* sqes = mmap(
		NULL,
		params.sq_entries * sizeof(struct io_uring_sqe),
		PROT_READ | PROT_WRITE,
		MAP_SHARED | MAP_POPULATE,
		file_desc,
		IORING_OFF_SQES
	);
	if (sqes == -1) return 1;



	unsigned int* sq_head = sq_ring + params.sq_off.head;
    unsigned int* sq_tail = sq_ring + params.sq_off.tail;
    unsigned int* sq_array = sq_ring + params.sq_off.array;

    unsigned int* cq_head = cq_ring + params.cq_off.head;
    unsigned int* cq_tail = cq_ring + params.cq_off.tail;
    struct io_uring_cqe* cqes = cq_ring + params.cq_off.cqes;

    // 3. preparar SQE para write
    const char msg[] = "Hello World\n";

    unsigned tail = *sq_tail;
    unsigned idx = tail & (params.sq_entries - 1);
    struct io_uring_sqe* sqe = &sqes[idx];

    memset(sqe, 0, sizeof(*sqe));

    sqe->opcode = IORING_OP_WRITEV;
    sqe->fd = STDOUT_FILENO;

    struct iovec iov = {
		(void*) msg,
		sizeof(msg) - 1,
	};

    sqe->addr = (unsigned long) &iov;
    sqe->len = 1;
    sqe->user_data = 42;

    sq_array[idx] = idx;
    *sq_tail = tail + 1;

    // 4. submit
    syscall(__NR_io_uring_enter, file_desc, 1, 0, IORING_ENTER_GETEVENTS, NULL);

    // 5. leer CQE
    unsigned head = *cq_head;
    if (head != *cq_tail) {
        struct io_uring_cqe *cqe = &cqes[head & (params.cq_entries-1)];
        // opcional: revisar resultado
        *cq_head = head + 1;
    }

    // 6. cerrar
    close(file_desc);

	return 0;
}
