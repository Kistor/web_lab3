// Фильтруем только по проектам - нужно убрать popup, когда активна страница сотрудников
isFilterPopupDisplay = true;

function showPopup() {
  if (isFilterPopupDisplay) {
    popupWrapper.style.display = 'flex';
    faider.style.display = 'flex';
  }
}

function hidePopup() {
  if (isFilterPopupDisplay) {
    popupWrapper.style.display = 'none';
    faider.style.display = 'none';
  }
}

function showProjectsForm() {
  projectsFormContainer.style.display = 'flex';
  faider.style.display = 'flex';
}

function hideProjectsForm() {
  projectsFormContainer.style.display = 'none';
  faider.style.display = 'none';
}

function showEmployersForm() {
  employersFormContainer.style.display = 'flex';
  faider.style.display = 'flex';
}

function hideEmployersForm() {
  employersFormContainer.style.display = 'none';
  faider.style.display = 'none';
}

function createTableHeaders(targetArr, targetHeaderText, isProjects = 1) {

  // Сначала очистим все элементы в DOM для переиспользования вёрстки
  while (tableHeadContainer.firstChild) {
    tableHeadContainer.removeChild(tableHeadContainer.firstChild);
  }
  while (tableBodyContainer.firstChild) {
    tableBodyContainer.removeChild(tableBodyContainer.firstChild);
  }

  // затем заполним поступившими данными
  headerText.textContent = targetHeaderText;
  for (i = 0; i < targetArr.length; i++) {
    tableDiv = document.createElement("div");
    tableDiv.textContent = targetArr[i];
    tableDiv.className = "tableTh";
    tableHeadContainer.appendChild(tableDiv);
  }

  if (isProjects) {
    createBtn.setAttribute("onClick", "showProjectsForm(); fillForm('create');");
  } else {
    createBtn.setAttribute("onClick", "showEmployersForm(); fillForm('create');");
  }
}

function fillTableFromApi(rowId, apiArr, mode) {

  tableTr = document.createElement("div");
  tableTr.className = "tableTr";
  tableTr.id = rowId;
  if (mode == "projects") {
    tableTr.setAttribute("onClick", "fillForm('edit'); showProjectsForm(); getProjectById('" + rowId + "')");
  } else if (mode == "employers") {
    tableTr.setAttribute("onClick", "fillForm('edit'); showEmployersForm(); getEmployerById('" + rowId + "')");
  }

  currentTableTr = tableBodyContainer.appendChild(tableTr);

  for (i = 0; i < apiArr.length; i++) {

    tableDiv = document.createElement("div");
    tableDiv.textContent = apiArr[i];
    tableDiv.className = "tableTh";
    currentTableTr.appendChild(tableDiv);
  }
}

function fillForm(mode) {
  if (mode == "create") {
    // очищаем формы от предыдущих значений при создании новой записи
    document.getElementById("formForCreateProject").reset()
    document.getElementById("formForCreateEmployer").reset()

    document.getElementById('projectsFormHeader').textContent = "Создать запись о проекте";
    document.getElementById('employersFormHeader').textContent = "Создать запись о сотруднике";
    document.getElementById('deleteBtnEmployers').style.display = 'none';
    document.getElementById('deleteBtnProjects').style.display = 'none';

    // парсим всех сотрудников, чтобы наполнить форму создания записи о проектах в полях "Руководитель проекта" и "Исполнители"
    getAndFillEmploersToCreationForm();

    document.getElementById('saveBtnProjects').setAttribute("onclick", "createProject(projectName.value, projectCompanyClient.value, projectCompanyContractor.value, Array.from(document.querySelectorAll('.projectsContractor:checked')).map(inputElement => inputElement.value),Array.from(document.querySelectorAll('#projectNameSelect')).map(inputElement => inputElement.value),projectPriority.value);formForCreateProject.reset()")
    document.getElementById('saveBtnEmployers').setAttribute("onclick", "createEmployer(employerName.value, employerSeconName.value, employerSurname.value, employerEmail.value, isProjectManagerSelect.value); formForCreateEmployer.reset()")

  } else if (mode == "edit") {
    document.getElementById('projectsFormHeader').textContent = "Редактировать запись о проекте";
    document.getElementById('employersFormHeader').textContent = "Редактировать запись о сотруднике";
    document.getElementById('deleteBtnEmployers').style.display = 'block';
    document.getElementById('deleteBtnProjects').style.display = 'block';

    document.getElementById('saveBtnEmployers').setAttribute("onclick", "updateEmployerById(openedEmployerId, employerName.value, employerSeconName.value, employerSurname.value, employerEmail.value, isProjectManagerSelect.value); formForCreateEmployer.reset()")
    document.getElementById('deleteBtnEmployers').setAttribute("onclick", "deleteEmployerById(openedEmployerId)")

    document.getElementById('saveBtnProjects').setAttribute("onclick", "updateProjectById(openedProjectId, projectName.value, projectCompanyClient.value, projectCompanyContractor.value, Array.from(document.querySelectorAll('.projectsContractor:checked')).map(inputElement => inputElement.value),Array.from(document.querySelectorAll('#projectNameSelect')).map(inputElement => inputElement.value),projectPriority.value);formForCreateProject.reset()")
    document.getElementById('deleteBtnProjects').setAttribute("onclick", "deleteProjectById(openedProjectId)")
  }
}

function getAndFillEmploersToCreationForm() {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/employee');

  xhr.open('GET', url);

  xhr.onload = () => {
    if (xhr.status == 200) {

      let employers = JSON.parse(xhr.responseText);

      // очищаем список ранее созданных сотрудников в форме, чтобы они не дублировались
      checkboxListContainer.textContent = ""
      p = document.createElement("p");
      p.textContent = "Исполнители"
      checkboxListContainer.appendChild(p)

      employers.response.forEach((currentEmployer) => {
        if (currentEmployer.isManager) {
          fillProjectsManagerToCreationForm(currentEmployer.id, currentEmployer.surname)
        }
        fillProjectsContractors(currentEmployer.id, currentEmployer.surname)
      });

    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send();
}

function getAndFillEmploersToEditionForm(employersOnProjectIdsArr, employersLidOnProjectIdsArr) {

  // очищаем список ранее созданных сотрудников в форме, чтобы они не дублировались
  checkboxListContainer.textContent = "";
  projectNameSelect.textContent = "";
  p = document.createElement("p");
  p.textContent = "Исполнители";
  checkboxListContainer.appendChild(p);

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/employee');

  xhr.open('GET', url);

  xhr.onload = () => {
    if (xhr.status == 200) {

      let employers = JSON.parse(xhr.responseText);

      employersLidOnProjectIdsArr.forEach((currentId) => {

        employers.response.forEach((currentEmployer) => {
          if (currentEmployer.id == currentId & currentEmployer.isManager) {
            fillProjectsManagerToCreationForm(currentEmployer.id, currentEmployer.surname)
          }
        })
      });

      employersOnProjectIdsArr.forEach((currentId) => {

        employers.response.forEach((currentEmployer) => {
          if (currentEmployer.id == currentId) {
            fillProjectsContractors(currentEmployer.id, currentEmployer.surname, true)
          }
        })
      });

    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send();
}

function fillProjectsManagerToCreationForm(id, surname) {
  option = document.createElement("option");
  option.value = id;
  option.textContent = surname;
  projectNameSelect.appendChild(option);
}

function fillProjectsContractors(id, surname, isNeedChecked = false) {

  contractorsContainer = document.createElement("div");
  contractorsContainer.className = "checkboxRowContainer";

  input = document.createElement("input");
  input.className = "pointer";
  input.className = "projectsContractor";
  input.type = "checkbox";
  input.value = id;
  input.id = id;
  // В форме редактирования записи делаем активными попавших в неё исполнителей для последующего редактирования
  if (isNeedChecked) input.checked = true
  contractorsContainer.appendChild(input);

  label = document.createElement("label");
  label.textContent = surname;
  label.setAttribute('for', id);
  contractorsContainer.appendChild(label);

  checkboxListContainer.appendChild(contractorsContainer)
}

function resetTable() {
  tableBodyContainer.textContent = ""
}

function formatDate(date) {

  var dd = date.getDate();
  if (dd < 10) dd = '0' + dd;

  var mm = date.getMonth() + 1;
  if (mm < 10) mm = '0' + mm;

  var yy = date.getFullYear();

  return yy + '-' + mm + '-' + dd;
}

// При запуске

// Для запуска в состояниии "Проекты", массив для наполнения берём из файла structs.js
createTableHeaders(projectsPageTableHeaders, 'Проекты', 1);
getAllProjects()

// Для запуска в состояниии "Сотрудники", массив для наполнения берём из файла structs.js
// createTableHeaders(employersPageTableHeaders, 'Сотрудники', 0);
// getAllEmployers()

// запоминаем ID сотрудника, для которого открыта форма редактирования записи
openedEmployerId = "";

// запоминаем ID проекта, для которого открыта форма редактирования записи
openedProjectId = "";

// API

function returnProjects() {
  const projects = {
    response: [
      {
        id: "d1219939-fecd-4647-9209-9554a9626091",
        nameProject: "Покупка коровы",
        nameCustomer: "ООО Моя оборона",
        namePerformer: "ИП Матроскин",
        employeeId: ["ds9676ed-43e5-4bf8-b50d-703a8d317efd", "653176ed-43e5-4bf8-b50d-703a8d317efd", "mb9676ed-43e5-4bf8-b50d-703a8d317ehh"],
        employeeLidId: ["ds9676ed-43e5-4bf8-b50d-703a8d317efd"],
        priority: 1,
        dateStart: 1617182400000,
        dateEnd: 1686565370000
      },
      {
        id: "0c2334f8-634a-41e7-900b-237978ebf091",
        nameProject: "Доставка писем",
        nameCustomer: "ООО Твоя оборона",
        namePerformer: "ИП Печкин",
        employeeId: ["mb9676ed-43e5-4bf8-b50d-703a8d317ehh", "653176ed-43e5-4bf8-b50d-703a8d317efd"],
        employeeLidId: ["653176ed-43e5-4bf8-b50d-703a8d317efd"],
        priority: 2,
        dateStart: 1699376402000,
        dateEnd: 1699525370000
      }
    ],
    success: true
  };
  return JSON.stringify(projects)
}

function returnProjectById(id) {
  if (id == "d1219939-fecd-4647-9209-9554a9626091") {
    const project = {
      response: {
        id: "d1219939-fecd-4647-9209-9554a9626091",
        nameProject: "Покупка коровы",
        nameCustomer: "ООО Моя оборона",
        namePerformer: "ИП Матроскин",
        employeeId: ["ds9676ed-43e5-4bf8-b50d-703a8d317efd", "653176ed-43e5-4bf8-b50d-703a8d317efd", "mb9676ed-43e5-4bf8-b50d-703a8d317ehh"],
        employeeLidId: ["ds9676ed-43e5-4bf8-b50d-703a8d317efd"],
        priority: 1,
        dateStart: 1617182400000,
        dateEnd: 1686565370000
      },
      success: true
    };
    return JSON.stringify(project)
  }
  else if (id == "0c2334f8-634a-41e7-900b-237978ebf091") {
    const project = {
      response: {
        id: "0c2334f8-634a-41e7-900b-237978ebf091",
        nameProject: "Доставка писем",
        nameCustomer: "ООО Твоя оборона",
        namePerformer: "ИП Печкин",
        employeeId: ["mb9676ed-43e5-4bf8-b50d-703a8d317ehh", "653176ed-43e5-4bf8-b50d-703a8d317efd"],
        employeeLidId: ["653176ed-43e5-4bf8-b50d-703a8d317efd"],
        priority: 2,
        dateStart: 1699376402000,
        dateEnd: 1699525370000
      },
      success: true
    };
    return JSON.stringify(project)
  }
}

// Сотрудники

async function getAllEmployers() {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/employee');

  xhr.open('GET', url);

  xhr.onload = () => {
    if (xhr.status == 200) {
      let employers = JSON.parse(xhr.responseText);

      // очищаем всю таблицу для чистого наполнения из API
      resetTable()

      employers.response.forEach((currentEmployer) => {
        let mappedIsManager = currentEmployer.isManager ? "Да" : "Нет";
        fillTableFromApi(currentEmployer.id, [currentEmployer.surname, currentEmployer.name, currentEmployer.secondName, currentEmployer.email, mappedIsManager], 'employers');
      });

    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send();

}

async function getEmployerById(id) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/employee/' + id);

  xhr.open('GET', url);

  xhr.onload = () => {
    if (xhr.status == 200) {
      let employer = JSON.parse(xhr.responseText);

      employerSurname.value = employer.response.surname;
      employerSurname.textContent = employer.response.surname;

      employerName.value = employer.response.name;
      employerName.textContent = employer.response.name;

      employerSeconName.value = employer.response.secondName;
      employerSeconName.textContent = employer.response.secondName;

      employerEmail.value = employer.response.email;
      employerEmail.textContent = employer.response.email;

      isProjectManagerSelect.value = employer.response.isManager;

      openedEmployerId = id;

    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send();

}

async function createEmployer(name, secondName, surname, email, isManager) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/employee/new');

  let processedIsManager = isManager == "true" ? true : false;

  let bodyForSend = JSON.stringify(
    {
      name: name,
      secondName: secondName,
      surname: surname,
      email: email,
      isManager: processedIsManager
    }
  );

  xhr.open("POST", url);

  xhr.setRequestHeader("Accept", "application/json");
  xhr.setRequestHeader("Content-Type", "application/json");

  xhr.onload = () => {
    if (xhr.status == 200) {
      getAllEmployers();
      hideEmployersForm();
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send(bodyForSend);

}

async function updateEmployerById(id, name, secondName, surname, email, isManager) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/employee/' + id);

  let processedIsManager = isManager == "true" ? true : false;

  let bodyForSend = JSON.stringify(
    {
      name: name,
      secondName: secondName,
      surname: surname,
      email: email,
      isManager: processedIsManager
    }
  );

  xhr.open("PATCH", url);

  xhr.setRequestHeader("Accept", "application/json");
  xhr.setRequestHeader("Content-Type", "application/json");

  xhr.onload = () => {
    if (xhr.status == 200) {
      getAllEmployers();
      hideEmployersForm();
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send(bodyForSend);

}

async function deleteEmployerById(id) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/employee/remote');

  let bodyForSend = JSON.stringify(
    {
      id: id
    }
  );

  xhr.open("DELETE", url);

  xhr.setRequestHeader("Accept", "application/json");
  xhr.setRequestHeader("Content-Type", "application/json");

  xhr.onload = () => {
    if (xhr.status == 200) {
      getAllEmployers();
      hideEmployersForm();
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send(bodyForSend);
}

// Проекты

async function getAllProjects() {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/projects');



  xhr.open('GET', url);
  xhr.setRequestHeader("Accept", "application/json");
  xhr.setRequestHeader("Content-Type", "application/json");




  xhr.onload = () => {
    if (xhr.status == 200) {
      let projects = JSON.parse(xhr.responseText);

      // очищаем всю таблицу для чистого наполнения из API
      resetTable()

      projects.response.forEach((currentProject) => {
        fillTableFromApi(currentProject.id, [currentProject.nameProject, currentProject.nameCustomer, currentProject.namePerformer, currentProject.priority], 'projects');
      });
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send();
}

async function getProjectById(id) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/projects/' + id);

  xhr.open('GET', url);

  xhr.onload = () => {
    if (xhr.status == 200) {
      let project = JSON.parse(xhr.responseText);

      projectName.value = project.response.nameProject;
      projectName.textContent = project.response.nameProject;

      projectPriority.value = project.response.priority;
      projectPriority.textContent = project.response.priority;

      getAndFillEmploersToEditionForm(project.response.employeeId, project.response.employeeLidId);

      projectCompanyClient.value = project.response.namePerformer;
      projectCompanyClient.textContent = project.response.namePerformer;

      projectCompanyContractor.value = project.response.nameCustomer;
      projectCompanyContractor.textContent = project.response.nameCustomer;

      let unixTimestampStart = project.response.dateStart
      let date = new Date(unixTimestampStart)
      projectDateStart.value = formatDate(date);

      let unixTimestampEnd = project.response.dateEnd
      let dateEnd = new Date(unixTimestampEnd)
      projectDateEnd.value = formatDate(dateEnd);

      openedProjectId = id;
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send();
}

async function createProject(nameProject, nameCustomer, namePerformer, employeeId, employeeLidId, priority) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/projects/new');

  let bodyForSend = JSON.stringify(
    {
      nameProject: nameProject,
      nameCustomer: nameCustomer,
      namePerformer: namePerformer,
      employeeId: employeeId,
      performers: employeeId[0],
      employeeLidId: employeeLidId,
      priority: parseInt(priority),
      dateStart: projectDateStart.valueAsNumber,
      dateEnd: projectDateEnd.valueAsNumber
    }
  );

  xhr.open("POST", url);

  xhr.setRequestHeader("Accept", "application/json");
  xhr.setRequestHeader("Content-Type", "application/json");

  xhr.onload = () => {
    if (xhr.status == 200) {
      getAllProjects();
      hideProjectsForm();
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send(bodyForSend);
}

async function updateProjectById(id, nameProject, nameCustomer, namePerformer, employeeId, employeeLidId, priority) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/projects/' + id);

  let bodyForSend = JSON.stringify(
    {
      id: id,
      nameProject: nameProject,
      nameCustomer: nameCustomer,
      namePerformer: namePerformer,
      employeeId: employeeId,
      performers: employeeId[0],
      employeeLidId: employeeLidId,
      priority: parseInt(priority),
      dateStart: projectDateStart.valueAsNumber,
      dateEnd: projectDateEnd.valueAsNumber
    }
  );

  xhr.open("PATCH", url);

  xhr.setRequestHeader("Accept", "application/json");
  xhr.setRequestHeader("Content-Type", "application/json");

  xhr.onload = () => {
    if (xhr.status == 200) {

      getAllProjects();
      hideProjectsForm();
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send(bodyForSend);
}

async function deleteProjectById(id) {

  let baseUrl = 'http://localhost:43836';
  let xhr = new XMLHttpRequest();

  let url = new URL(baseUrl + '/projects/remote');

  let bodyForSend = JSON.stringify(
    {
      id: id
    }
  );

  xhr.open("DELETE", url);

  xhr.setRequestHeader("Accept", "application/json");
  xhr.setRequestHeader("Content-Type", "application/json");

  xhr.onload = () => {
    if (xhr.status == 200) {
      getAllProjects();
      hideProjectsForm();
    } else {
      console.log("Server response: ", xhr.statusText);
    }
  };

  xhr.send(bodyForSend);
}
