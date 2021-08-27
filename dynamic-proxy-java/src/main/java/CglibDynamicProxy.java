import net.sf.cglib.proxy.Enhancer;
import net.sf.cglib.proxy.MethodInterceptor;
import net.sf.cglib.proxy.MethodProxy;

import java.lang.reflect.Method;

public class CglibDynamicProxy {
    public static void main(String[] args) {

        CglibTarget cglibTarget = new CglibTarget();
        CglibDynamicProxy cglibDynamicProxy = new CglibDynamicProxy();
        CglibTarget proxy = (CglibTarget) cglibDynamicProxy.createProxy(cglibTarget.getClass());
        proxy.executor();
    }
    
    public Object createProxy(Class clazz){
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(clazz);
        enhancer.setCallback(new CglibMethodIntercept());
        return enhancer.create();
    }
}

class CglibTarget{
    public void executor(){
        System.out.println("执行了executor方法……");
    }
}

// MethodInterceptor接口继承了Callback空接口
class CglibMethodIntercept implements MethodInterceptor{

    @Override
    public Object intercept(Object o, Method method, Object[] args, MethodProxy methodProxy) throws Throwable {
        System.out.println("Cglib代理开始执行……");
        Object object = methodProxy.invokeSuper(o, args);
        System.out.println("Cglib代理执行结束……");
        return object;
    }
}
