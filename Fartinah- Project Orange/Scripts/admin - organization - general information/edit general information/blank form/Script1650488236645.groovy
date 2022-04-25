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

WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/span_Username'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_LOGIN Panel_txtUsername'), 
    'Admin')

WebUI.setEncryptedText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_Username_txtPassword'), 
    'hUKwJTbofgPU9eVlw/CnDQ==')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_Password_Submit'))

WebUI.click(findTestObject('HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/b_Admin'))

WebUI.click(findTestObject('HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/a_Organization'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/a_General Information'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input__btnSaveGenInfo'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/ol_Organization Name                       _7572cb'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input__organizationname'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/ol_Organization Name                       _261ace'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_Tax ID_organizationtaxId'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/div_Organization Name                      _34bf95'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_Phone_organizationphone'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/div_Organization Name                      _34bf95'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_Email_organizationemail'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/li_Address Street 1'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_Address Street 1_organizationstreet1'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/li_City'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_City_organizationcity'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/li_Note                        HRM Software'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/textarea_HRM Software'), 
    '')

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_StateProvince_organizationprovince'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/li_StateProvince'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input_StateProvince_organizationprovince'), 
    '')

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/General Infromation/Page_OrangeHRM/input__btnSaveGenInfo'))

