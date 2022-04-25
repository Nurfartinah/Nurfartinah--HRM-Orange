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

WebUI.callTestCase(findTestCase('Common/Login HRM web'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('HRM ORANGE/Admin/Page_OrangeHRM/b_Admin'))

WebUI.click(findTestObject('HRM ORANGE/Admin/Page_OrangeHRM/a_Job'))

WebUI.click(findTestObject('HRM ORANGE/Admin/Page_OrangeHRM/a_Work Shifts'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/input_Work Shifts_btnAdd (1)'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/input__workShiftname (1)'), 
    'Shift for malam raya')

WebUI.selectOptionByValue(findTestObject('HRM ORANGE/Admin/Page_OrangeHRM/select_000000150030004501000115013001450200_bef9c6_1'), 
    '25', true)

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/a_Add'))

WebUI.selectOptionByValue(findTestObject('HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/select_Garry White'), '23', true)

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/a_Add'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/input__btnSave'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/input_General_chkSelectRow_1'))

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/a_Shift for malam raya'))

WebUI.setText(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/input__workShiftname (1)'), 
    'Shift for malam raya korban')

WebUI.selectOptionByValue(findTestObject('HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/select_000000150030004501000115013001450200_bef9c6 (1)'), 
    '05:00', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/select_000000150030004501000115013001450200_bef9c6_1 (1)'), 
    '12:45', true)

WebUI.click(findTestObject('Object Repository/HRM ORANGE/Admin/Page_OrangeHRM/Page_OrangeHRM/input__btnSave'))

