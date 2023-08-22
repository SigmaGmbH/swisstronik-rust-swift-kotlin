package com.swisstronik.kotlin

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.swisstronik.kotlin.AndroidTestUtils.decodeHex
import com.swisstronik.kotlin.AndroidTestUtils.toHexString
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class SwisstronikTest {
    @Test
    fun useAppContext() {
        // Context of the app under test.
        val appContext = InstrumentationRegistry.getInstrumentation().targetContext
        assertEquals("com.swisstronik.kotlin", appContext.packageName)
    }

    @Test
    fun initLogger() {
        SwisstronikKotlin().initLogger()
        // There's no result. Only testing that it doesn't crash.
    }
    @Test
    fun testEncryptDecrypt() {
        val ffiSystem = SwisstronikKotlin()
        ffiSystem.initLogger()
        val userPrivateKey = "C516DC17D909EFBB64A0C4A9EE1720E10D47C1BF3590A257D86EEB5FFC644D43".decodeHex()
        val nodePublicKey = "86477673c1c6fd9d061e884f56d440b2ce03fa2fe39a2a4882357a451a7f490e".decodeHex()
        val plaintext = kotlin.random.Random.nextBytes(128)
        println("plaintext - ${plaintext.toHexString()}")
        val encrypted = ffiSystem.swisstronikEncrypt(userPrivateKey,nodePublicKey,plaintext).getOrThrow()
        println("encrypted - ${encrypted.toHexString()}")
        val decrypted = ffiSystem.swisstronikDecrypt(userPrivateKey, nodePublicKey, encrypted)
        println("decrypted - ${decrypted.getOrThrow().toHexString()}")
        assertEquals(decrypted.getOrThrow().toHexString(),plaintext.toHexString())
    }

}