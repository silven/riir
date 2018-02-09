#include <stdio.h>
#include <math.h>
#include <complex.h>

typedef double complex cplx;

void _fft(cplx buf[], cplx out[], size_t n, size_t step)
{
	if (step < n) {
		_fft(out, buf, n, step * 2);
		_fft(out + step, buf + step, n, step * 2);
 
		for (size_t i = 0; i < n; i += 2 * step) {
			cplx t = cexp(-I * M_PI * i / n) * out[i + step];
			buf[i / 2]     = out[i] + t;
			buf[(i + n)/2] = out[i] - t;
		}
	}
}
 
void fft(cplx buf[], size_t n)
{
	cplx out[n];
	for (size_t i = 0; i < n; i++) {
        out[i] = buf[i];
    }
	_fft(buf, out, n, 1);
}
 
 
void show(const char * s, cplx buf[], size_t n) {
	printf("%s", s);
	for (size_t i = 0; i < n; i++) {
		if (!cimag(buf[i])) {
			printf("%g ", creal(buf[i]));
        } else {
			printf("(%g, %g) ", creal(buf[i]), cimag(buf[i]));
        }
    }
    printf("\n");
}
