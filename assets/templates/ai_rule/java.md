## Java 项目规则

### 构建工具（二选一，遵循项目现状）

项目使用 Maven 或 Gradle，**不得混用**：

- Maven：`pom.xml` + `mvn` 命令
- Gradle：`build.gradle` 或 `build.gradle.kts` + `gradle` / `gradlew` 命令

### 运行时

1. **JDK 版本必须与 `pom.xml` 的 `<maven.compiler.release>` 或 `build.gradle` 的 `sourceCompatibility` 一致**
2. 不得擅自升级或降级 JDK target
3. Spring Boot 项目必须保留主版本一致性（如 Spring Boot 3.x 要求 JDK 17+）

### 项目结构约定（Maven Standard Layout）

遵循 Maven 标准目录结构，不得擅自调整一级目录：

- `src/main/java/` — 主代码（按包名分层）
  - `com/<org>/<project>/`
    - `controller/` — REST 控制器（Spring）
    - `service/` — 业务逻辑
    - `repository/` 或 `dao/` — 数据访问层
    - `model/` 或 `entity/` — 实体类
    - `config/` — 配置类
    - `dto/` — 数据传输对象
- `src/main/resources/` — 配置文件（application.yml / application.properties）
- `src/test/java/` — 单元测试（镜像 main 结构）
- `src/test/resources/` — 测试资源

### 构建与测试命令

Maven：
- `mvn clean install` — 清理并安装
- `mvn compile` — 编译
- `mvn test` — 运行测试
- `mvn package` — 打包
- `mvn spring-boot:run` — 运行 Spring Boot（如适用）

Gradle：
- `./gradlew build` — 构建
- `./gradlew test` — 运行测试
- `./gradlew bootRun` — 运行 Spring Boot（如适用）

### 依赖与配置规则

1. `pom.xml` / `build.gradle` 扩展依赖，不得重写整个文件
2. 依赖版本管理：Maven 通过 `<dependencyManagement>`，Gradle 通过版本目录或 ext 变量，遵循项目现状
3. Spring Boot 项目使用 `@Autowired` / 构造器注入，不得在内部 `new` Service
4. 配置文件修改采取合并策略，不破坏现有键值
5. JPA / MyBatis / Hibernate 等 ORM 遵循项目现状，不得擅自替换

### 禁止事项

- 不得在 Maven 项目中创建 `build.gradle`（或反之）
- 不得擅自升级或降级 JDK / Spring Boot 主版本
- 不得在 Controller 中编写业务逻辑（应放在 Service）
- 不得破坏包结构分层（如把 Service 放到 controller 包）
- 不得擅自切换 ORM 框架（JPA ↔ MyBatis）
- 不得引入与项目 Lombok 配置冲突的依赖
