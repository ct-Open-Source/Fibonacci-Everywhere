import java.math.*; 

public class fibonacci{ 
    public static void printFibonacci(int tries){ 
        BigInteger a = new BigInteger("0"); 
        BigInteger b = new BigInteger("1");
        for (int i = 0; i < tries; i = i + 1){
            System.out.println(a); 
            BigInteger c = a.add(b); 
            a = b; 
            b = c; 
        } 
    } 
        
    public static void main(String[] args){  
        printFibonacci(10); 
    } 
}
