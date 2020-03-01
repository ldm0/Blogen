LaTeX Compatibility Test 
2020/1/29  
Latex | Test  

---

拿不知道什么时候的快速傅里叶介绍来测试LaTeX

---

##### 算法设计与实践

### 快速傅里叶变换

+ 内容
    1. 离散傅里叶变换(Discrete Fourier Transform)
        1. 一组特殊的复数
            + 对于多项式$A(x)=a_0 + a_1*x^1 + a_2*x^2 + ... + a_{n-1}*x^{n-1}$，离散傅里叶变换要求我们将$n$次单位根的$0$到$n$次幂分别代入$A(x)$得到结果点值数组$(A(\omega_n^0),A(\omega_n^1),A(\omega_n^2),...,A(\omega_n^{n-1}))$
            + 其中这$n$个单位根是一组特殊的复数。在复平面上这n个单位根是单位圆的n等分的切割射线与单位圆的交点。
    2. 快速傅里叶变换(Fast Fourier Transform)
        1. 有意思的上面这组特殊的复数有一些神奇的性质
        2. 因为它们有着一些神奇的性质
            1. $\omega_{2n}^{2k}=\omega_{n}^{k}$
            2. $\omega_{n}^{k+\frac{n}{2}}=-\omega_n^k$
        3. 而这些性质在计算的时候运用一些神奇的技巧的话会有神奇加速效用
            1. 对于$A(x)=a_0 + a_1*x^1 + a_2*x^2 + ... + a_{n-1}*x^{n-1}$，我们进行奇偶分组，得到:
                $$A(x)=(a_0 + a_2*x^2 + a_4*x^4 + ... + a_{n-2}*x^{n-2}) + (a_1*x^1 + a_3*x^3 + a_5*x^5 + ... + a_{n-1}*x^{n-1})$$
                $$A(x)=(a_0 + a_2*x^2 + a_4*x^4 + ... + a_{n-2}*x^{n-2}) + x * (a_1 + a_3*x^2 + a_5*x^4 + ... + a_{n-1}*x^{n-2})$$
            2. 令
                $$A1(x)=(a_0 + a_2*x^1 + a_4*x^2 + ... + a_{n-2}*x^{\frac{n-2}{2}})$$
                $$A2(x)=(a_1 + a_3*x^1 + a_5*x^2 + ... + a_{n-1}*x^{\frac{n-2}{2}})$$
            3. 我们得到:
                $$A(x) = A1(x^2) + x*A2(x^2)$$
            4. 由前面介绍的两个性质，通过计算得到如下的两个等式：
                + 当$0<k<\frac{n}{2}-1$时，有
                    1. $A(\omega_n^k)=A1(\omega_{\frac{n}{2}}^{k})+\omega_n^k * A2(\omega_{\frac{n}{2}}^{k})$
                    2. $A(\omega_n^{k+\frac{n}{2}})=A1(\omega_{\frac{n}{2}}^{k})-\omega_n^k * A2(\omega_{\frac{n}{2}}^{k})$
            5. 这样我们就能分治地进行离散傅里叶变换。时间复杂度满足$T(n) =2T(\frac{n}{2}) + O(n) = O(n\log{n})$
    + 应用
        + 大整数乘法
            + 大整数乘法算是快速傅里叶变换的一个重要的应用
            + 为了使用FFT来加速傅里叶变换，我们还需要引入另一个计算方法
                + 逆离散傅里叶变换(Inverse Discrete Fourier Transform)
                    1. 之前我们知道系数数组并通过计算得到了点值数组$(A(\omega_n^0),A(\omega_n^1),A(\omega_n^2),...,A(\omega_n^{n-1}))$，现在我们是要通过点值数组来反推系数数组。
                    2. 构造如下多项式:
                        $$F(x)=d_0+d_1*x^1 + d_2*x^2+...+d_{n-1}*x^{n-1}$$
                    3. 我们使用$(\omega_n^{-0}, \omega_n^{-1}, \omega_n^{-2},..., \omega_n^{-n+1})$代入，可以证明的是，得到的正好是如下结果：
                        $$(n*a_0, n * a_1, n*a_2,...,n*a_{n-1})$$
                    4. 最后，我们只要将结果乘以$\frac{1}{n}$，就得到了我们想要的系数数组：$(a_0, a_1, a_2,...,a_{n-1})$
            + 至此所有的计算工具已经完全，我们怎么使用快速傅里叶变换来加速大整数乘法呢?
                + 正常的算法是对于两个参数序列逐位计算，众所周知的计算复杂度为$O(n^2)$
                + 我们这时灵机一动，可以通过
                    1. 快速傅里叶变换将两个待运算参数数组转化成点值数组(时间复杂度$O(n\log{n})$)
                    2. 然后将点值数组相乘(时间复杂度$O(n)$)
                    3. 然后再使用逆傅里叶变换将相乘后得到的点值数组转换成结果的参数数组(时间复杂度$O(n\log{n})$)
                + 这样我们就能在$O(n\log{n})$的时间复杂度下完成两个大整数的乘法。
+ 大整数乘法pseudo code 
```rust
void core(array[n]) {
    rearange(array)
    for (i = 2; i < n; i <<= 1) {
        Complex step(cos(2 * PI / i), flag * sin(2 * PI / i))
        for k in (0..i) {
            Complex sweep(1, 0)
            for j in (0..(i / 2)) {
                x = array[k + j]
                y = sweep * array[k + j + (i / 2)]
                array[k + j] = x + y
                array[k + j + i / 2] = x - y
                sweep *= step
            }
        }
    }
}

void fft(array[n]) {
    core(array, 1)
}

void ifft(array[n]) {
    core(array, -1)
    for i in (0..n) {
        array[i] /= n
    }
}

void bigmulti(a[n], b[n]) {
    amulb[n];
    result[n];
    fft(a)
    fft(b)
    for i in (0..n) {
        amulb[i] = a[i] * b[i];
    }
    ifft(amulb, result)
    print(reuslt.real_part)
}
```
