import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://testops.katalon.io/')

WebUI.click(findTestObject('Object Repository/Page_Katalon TestOps/div_Sign InSign UpBy signing in to Katalon _2d7b60'))

WebUI.setText(findTestObject('Object Repository/Page_Katalon TestOps/input_Sign Up_username'), 'duong.kieu@katalon.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Katalon TestOps/input_Sign Up_password'), '9Sd1Uh1O/N8=')

WebUI.click(findTestObject('Object Repository/Page_Katalon TestOps/button_Sign in'))

WebUI.setViewPortSize(1920, 1003)

WebUI.takeScreenshotAsCheckpoint('Login Flow Test Run')

WebUI.closeBrowser()

