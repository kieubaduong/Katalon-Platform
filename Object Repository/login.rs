<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>login</name>
   <tag></tag>
   <elementGuidId>b7a9b161-9400-42ef-ab48-53b63f27b45b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsia2l0Il0sImthdE9uZUp3dCI6ImV5SjBlWEFpT2lKS1YxUWlMQ0poYkdjaU9pSlNVekkxTmlKOS5leUp6ZFdJaU9pSmtkVzl1Wnk1cmFXVjFRR3RoZEdGc2IyNHVZMjl0SWl3aWRYQnVJam9pTVRJMk1EQXpNQ0lzSW1keWIzVndjeUk2V3lKVlUwVlNJbDBzSW5OMFlYUjFjeUk2SWxaRlVrbEdTVVZFSWl3aVoybDJaVzVmYm1GdFpTSTZJa3RwWlhVaUxDSm1ZVzFwYkhsZmJtRnRaU0k2SWtKaElFUjFiMjVuSWl3aVpXMWhhV3dpT2lKa2RXOXVaeTVyYVdWMVFHdGhkR0ZzYjI0dVkyOXRJaXdpWlcxaGFXeGZkbVZ5YVdacFpXUWlPblJ5ZFdVc0ltTnlaV0YwWldSZllYUWlPakUyT0Rnek5qVTVOamdzSW5Wd1pHRjBaV1JmWVhRaU9qRTJPRGc1TnpVNU1UZ3NJbUZ0Y2lJNld5SndkMlFpWFN3aWFXRjBJam94TmpnNE9UYzFPVFV6TENKbGVIQWlPakUyT0RrMU9EQTNOVE1zSW1wMGFTSTZJamsyTm1WbE56azJMVEEwTW1RdE5EUm1aQzFpWTJJMExUSTVNbU13TmpKaVlUbG1PQ0lzSW1semN5STZJbXRoZEdGc2IyNHRZWFYwYUNKOS5tUndyRW9zN1o2QmhZa1F5OVFxVFdkd0xTNDcteVFQT01EX2lvd0VrWUl4WVl2T0lYc0VxNFhMc2JqcnN4bWZ3YW9JaWZFWk9Yck1MREk0Z2JYeVZaUEQyaERHbG1aVmozdUxrTlNYYXZRdlVuOXhQeEYtcW9kM1dEUjJ1NHFvNTRwUFJLUXFKV3lGVUl3ZjMyU2RuOGh5dl9KZFFfRk9Sbnd1eVg3VHRaVWt2MTNoancyVWpvdHZ0MmpBQm9JeDRiSUxWeTF0SmdpZ3BmMzg0NGpMc1RreWhzcEF6Z29hLXpnTXZTZU1najdET25UTkxQVFpkcDllRmIzM0lESzRUMkRfVDVYQ05WMktwTDJaV2Z5eU1sVnN6bUpEVk5ra3RDbTVZSXZ1Z2tNX1R3OGh2WHVKWEtYVFlaRE1xamNERzNfQU9qTWEycXAxMlBVLTRLb28yOHciLCJ1c2VyX25hbWUiOiJkdW9uZy5raWV1QGthdGFsb24uY29tIiwic2NvcGUiOlsicmVhZCIsIndyaXRlIl0sImthdE9uZVVzZXJJZCI6MTI2MDAzMCwiZXhwIjoxNjg5NTgwNzUzLCJqdGkiOiJiNmY5YjMyZS00ZTNkLTRkZDgtYjAyYy0wOGQ0NzcyNzE0NTYiLCJjbGllbnRfaWQiOiJraXQifQ.TVVXctH1x3tsrzL5jt-nEjpIu8HSybUjGK9MjOPN9VIp_pw5qgnNvsxAaNWVpsWFFGFkVlmpwQOfmyJXwtjZLUa-Uy6_3X0rXf1bClQSvvGKFKeEZ77kW44D3w7jjNnpktFPlCtpGRyANTW8pfONs6BeCY-4GQXOAtN0sKsKeik2brqJpIH2THNytbh914tdnWguaWx3vXu62KwxtvI0oqJBhmueKy8hzldLb2fLo7mbuGPCEHcdfGZDyrXAlzT1SMOjRvUA-xoIfwYIlEtq0TUY1k58mNTM942gx7wA7NgzdunXLZY74cQlnS-FwC-FKpmxNOCAUKMZ5owhfK3zkQ</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;username&quot;,
      &quot;value&quot;: &quot;duong.kieu@katalon.com&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;${GlobalVariable.password}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;grant_type&quot;,
      &quot;value&quot;: &quot;password&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;webLogin&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>ed2a47fe-b1e0-4586-99d6-b2dcc5195d89</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.base_url}/${GlobalVariable.login_url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.username</defaultValue>
      <description></description>
      <id>9af8e51a-bf65-4680-8e46-db72d158e51c</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>37c18a62-495f-4303-8bbd-a9652264de46</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
