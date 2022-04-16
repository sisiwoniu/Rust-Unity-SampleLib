using System;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.Profiling;

public class RustPluginTester : MonoBehaviour {
#if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    [DllImport("__Internal")]
#else
    [DllImport("UnitySamplePlugin")]
#endif
    static extern int add(int a, int b);

#if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    [DllImport("__Internal")]
#else
    [DllImport("UnitySamplePlugin")]
#endif
    unsafe static extern int * add_ver_2(int * src);

    // #if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    //     [DllImport("__Internal")]
    // #else
    //     [DllImport("UnitySamplePlugin")]
    // #endif
    //     unsafe static extern void test_fn_1(int * src, int length);

#if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    [DllImport("__Internal")]
#else
    [DllImport("UnitySamplePlugin")]
#endif
    unsafe static extern void delete_ptr(int * src);

#if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    [DllImport("__Internal")]
#else
    [DllImport("UnitySamplePlugin")]
#endif
    unsafe static extern void test_fn_1(IntPtr src, int length);

#if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    [DllImport("__Internal")]
#else
    [DllImport("UnitySamplePlugin")]
#endif
    unsafe static extern IntPtr test_fn_2(IntPtr src);

#if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    [DllImport("__Internal")]
#else
    [DllImport("UnitySamplePlugin")]
#endif
    static extern void test_fn_5([In, Out] TestStruct[] src, int length);

    [DllImport("UnitySamplePlugin")]
    unsafe static extern void delete_ptr(IntPtr src);

    [DllImport("UnitySamplePlugin")]
    static extern void test_fn_3(ref TestStruct src);

    [DllImport("UnitySamplePlugin")]
    static extern TestStruct test_fn_4(TestStruct src);

    private int[] changeNumArray = new int[] {
        1,
        3,
        4,
        5
    };

    struct TestStruct {
        public int X;

        public TestStruct(int x) {
            X = x;
        }

        public void LogOutAddress() {
            unsafe {
                fixed(int * p = & X) {
                    Debug.Log("アドレス: = " + (int)p);
                }
            }
        }
    }

    // Start is called before the first frame update
    void Start() {

    }

    // Update is called once per frame
    void Update() {
        // if (Input.GetKeyDown(KeyCode.A)) {
        //     var a = Random.Range(1, 3);
        //     var b = Random.Range(3, 5);
        //     var c = add(a, b);
        //     Debug.Log($"{a} + {b} = {c}");
        // }

        // if (Input.GetKeyDown(KeyCode.C)) {
        //     Test();
        // }

        if (Input.GetKeyDown(KeyCode.D)) {
            var testStruct1 = new TestStruct(1);

            var testStruct1Arr = new TestStruct[3] {
                new TestStruct(2),
                new TestStruct(3),
                new TestStruct(4)
            };

            unsafe {
                void Sample1(IntPtr ptr) {
                    // IntPtr testStruct1Ptr = Marshal.AllocCoTaskMem(Marshal.SizeOf<TestStruct>());
                    // Marshal.StructureToPtr(testStruct1, testStruct1Ptr, false);
                    // testStruct1.LogOutAddress();
                    //结果是debugPtr == testStruct1Ptr 说明Rust的Box转换并没有占用新的内存，而是直接使用原pointer然后再返还回来
                    /*var debugPtr =*/
                    test_fn_2( /*testStruct1Ptr*/ ptr);
                    // TestStruct result = Marshal.PtrToStructure<TestStruct>(testStruct1Ptr);
                    //marshal和delete_ptr都会解放pointer，所以不可以双重叫出
                    // Marshal.FreeCoTaskMem(testStruct1Ptr);
                    // delete_ptr(debugPtr);
                }

                Profiler.BeginSample("Sample Test Rust use pointer");
                int i = 100000;

                IntPtr testStruct1Ptr = Marshal.AllocCoTaskMem(Marshal.SizeOf<TestStruct>());
                Marshal.StructureToPtr(testStruct1, testStruct1Ptr, false);

                while (i > 0) {
                    Sample1(testStruct1Ptr);
                    i -= 1;
                }

                TestStruct result = Marshal.PtrToStructure<TestStruct>(testStruct1Ptr);

                delete_ptr(testStruct1Ptr);

                Profiler.EndSample();

                Profiler.BeginSample("Sample Test c# use pointer");
                i = 100000;
                while (i > 0) {
                    var value = 10000f;
                    for (int y = 0; y < 4; y++) {
                        value = Mathf.Sqrt(value);
                    }
                    TestStruct result2 = new TestStruct(Mathf.FloorToInt(value));
                    testStruct1 = result;
                    i -= 1;
                }
                // Debug.Log("result X == " + testStruct1.X);
                Profiler.EndSample();

                #region test array struct

                var structArray1 = new TestStruct[] {
                    new TestStruct(1),
                    new TestStruct(2),
                    new TestStruct(3),
                    new TestStruct(4),
                };

                var structArray2 = new TestStruct[] {
                    new TestStruct(1),
                    new TestStruct(2),
                    new TestStruct(3),
                    new TestStruct(4),
                };

                i = 100000;

                Profiler.BeginSample("Sample Test Rust Use Array");

                while (i > 0) {
                    test_fn_5(structArray1, structArray1.Length);
                    i -= 1;
                }

                Profiler.EndSample();

                Profiler.BeginSample("Sample Test c# Use Array");

                i = 100000;

                while (i > 0) {
                    for (int y = 0; y < 4; y++) {
                        var value = 10000f;
                        for (int z = 0; z < 4; z++) {
                            value = Mathf.Sqrt(value);
                        }
                        var d = structArray2[y];
                        d.X = Mathf.FloorToInt(value);
                        structArray2[y] = d;
                    }

                    i -= 1;
                }

                Profiler.EndSample();

                #endregion
                #region ref type
                void Sample3(ref TestStruct src) {
                    test_fn_3(ref src);
                }

                Profiler.BeginSample("Sample Test Rust use ref");
                i = 100000;

                var result3 = testStruct1;

                while (i > 0) {
                    Sample3(ref result);
                    i -= 1;
                }

                Profiler.EndSample();

                Profiler.BeginSample("Sample Test c# use ref");
                i = 100000;
                while (i > 0) {
                    var value = 10000f;
                    for (int y = 0; y < 4; y++) {
                        value = Mathf.Sqrt(value);
                    }
                    TestStruct result4 = new TestStruct(Mathf.FloorToInt(value));
                    testStruct1 = result4;
                    i -= 1;
                }
                Profiler.EndSample();
                #endregion

                #region copy type
                TestStruct Sample4(TestStruct src) {
                    return test_fn_4(src);
                }

                Profiler.BeginSample("Sample Test Rust use copy");
                i = 100000;

                var result5 = testStruct1;

                while (i > 0) {
                    result5 = Sample4(result5);
                    i -= 1;
                }

                Profiler.EndSample();

                Profiler.BeginSample("Sample Test c# use copy");
                i = 100000;
                while (i > 0) {
                    var value = 10000f;
                    for (int y = 0; y < 4; y++) {
                        value = Mathf.Sqrt(value);
                    }
                    TestStruct result6 = new TestStruct(Mathf.FloorToInt(value));
                    testStruct1 = result6;
                    i -= 1;
                }
                Profiler.EndSample();
                #endregion
            }

            // foreach (var num in changeNumArray) {
            //     Debug.Log("num1 == " + num);
            // }
            // unsafe {
            //     for (int i = 0; i < changeNumArray.Length; i++) {
            //         fixed(int * p = & changeNumArray[i]) {
            //             Debug.Log($"1 p[{i}] == {changeNumArray[i]}; address = {(int)p}");
            //         }
            //     }

            //     fixed(int * p = changeNumArray) {
            //         test_fn_1(p, changeNumArray.Length);
            //         for (int i = 0; i < changeNumArray.Length; i++) {
            //             Debug.Log($"p[{i}] == {p[i]};");
            //         }
            //     }

            //     for (int i = 0; i < changeNumArray.Length; i++) {
            //         fixed(int * p = & changeNumArray[i]) {
            //             Debug.Log($"2 p[{i}] == {changeNumArray[i]}; address = {(int)p}");
            //         }
            //     }
            // }

            // foreach (var num in changeNumArray) {
            //     Debug.Log("num == " + num);
            // }
        }
    }

    // private unsafe void Test() {
    //     string test = "test";
    //     fixed(char * p = test) {
    //         for (int i = 0; i < test.Length; i++) {
    //             Debug.Log("変更前: " + * p);
    //             p[i] = (char)(i + '1');
    //             Debug.Log("変更後: " + * p);
    //         }
    //     }
    //     Debug.Log("result == " + test);
    // }
}