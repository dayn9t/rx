# MQTT

paho_mqtt 库
- 不依赖 paho-mqtt-c ✅
- 接口简单 ✅
- 构建依赖: ```apt install libssl-dev build-essential cmake```

rumqttc 库
- 纯 Rust 实现
- 接口使用不便, 必须手动介入事件循环 ❌
- 支持异步 ✅
