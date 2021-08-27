import java.lang.reflect.InvocationHandler;
import java.lang.reflect.Method;
import java.lang.reflect.Proxy;

public class JdkDynamicProxy {

    public static void main(String[] args) {
        TargetImpl target = new TargetImpl();
        JdkProxyHandle jdkProxyHandle = new JdkProxyHandle(target);
        Target proxyInstance = (Target) Proxy.newProxyInstance(JdkDynamicProxy.class.getClassLoader(),
                target.getClass().getInterfaces(), jdkProxyHandle);
        proxyInstance.executor();
    }
}

interface Target{
    void executor();
}

class TargetImpl implements Target {

    @Override
    public void executor() {
        System.out.println("执行了executor……");
    }
}

class JdkProxyHandle implements InvocationHandler{

    private Target target;
    public JdkProxyHandle(Target target){this.target = target;}

    @Override
    public Object invoke(Object proxy, Method method, Object[] args) throws Throwable {

        System.out.println("进入代理执行目标方法前……");
        Object invoke = method.invoke(target, args);
        System.out.println("执行代理执行目标方法后……");

        return invoke;
    }
}
