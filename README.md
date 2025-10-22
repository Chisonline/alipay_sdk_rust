# [alipay_sdk_rust 异步修复版](https://github.com/Chisonline/alipay_sdk_rust)

Alipay sdk in rust 支付宝支付 SDK。只支持RSA2公钥证书方式签名验证方式, 默认只支持utf-8编码和JSON格式。目前只支持商户直接接入模式

 RSA2密钥生成请参考<https://opendocs.alipay.com/common/02kipl> 中的公钥证书方式生成，使用CSR文件申请，密钥格式必须使用PKCS1(非java适用)

 # 异步版本，生产可用

 ## 相比[原版](https://github.com/wandercn/alipay_sdk_rust)修改

 - [x] 去除了gostd的依赖，改用rust生态
 - [x] 默认使用tokio运行时
 - [x] 修复了部分bug，现在订单码支付可以正确支持

## TODO

 - [ ] 修复证书sn提取部分
 - [ ] 支持其他异步运行时，以及提供同步feature
