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
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import groovy.json.JsonSlurper as JsonSlurper

ResponseObject res = WS.sendRequest(findTestObject('login', [('username') : GlobalVariable.username, ('password') : GlobalVariable.password]))

def jsonSlurper = new JsonSlurper()

def jsonObject = jsonSlurper.parseText(res.getResponseBodyContent())

String accessToken = jsonObject.access_token

GlobalVariable.access_token = accessToken

res = WS.sendRequest(findTestObject('get_user_info'))

jsonObject = jsonSlurper.parseText(res.getResponseBodyContent())

def projects = jsonObject.projects

String targetName = 'Katalon Platform'

def projectId = null

def teamId = null

projects.each({ def obj ->
        if (obj.name == targetName) {
            projectId = obj.id

            teamId = obj.teamId

            return 
        }
    })

GlobalVariable.project_id = projectId

GlobalVariable.team_id = teamId

String temp = projectId.toString()

String queryProjectsParam = '{"type":"RunConfiguration","conditions":[{"key":"Project.id","operator":"=","value":' + temp + '}],"pagination":{"page":0,"size":30,"sorts":["updatedAt,desc"]}}'

println queryProjectsParam

GlobalVariable.queryProjectsParam = queryProjectsParam

res = WS.sendRequest(findTestObject('get_test_run_list'))

println jsonSlurper.parseText(res.getResponseBodyContent())

//def content = jsonSlurper.parseText(res.getResponseBodyContent()).content
//
//GlobalVariable.schedule_test_id = content[0].id
//
//res = WS.sendRequest(findTestObject('execute_schedule_test'))
//
//println jsonSlurper.parseText(res.getResponseBodyContent())









