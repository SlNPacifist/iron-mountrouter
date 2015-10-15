var searchIndex = {};
searchIndex['iron_mountrouter'] = {"items":[[0,"","iron_mountrouter","",null,null],[3,"MethodPicker","","`MethodPicker` provides an interface to apply different handlers for different http methods as\nmiddleware for the Iron framework.",null,null],[3,"NoMethod","","`NoMethod` is error type returned to Iron if no handler was matched in MethodPicker",null,null],[3,"Router","","`Router` provides an interface to apply different handlers to different routes as middleware for\nthe Iron framework.",null,null],[3,"StrippedUrl","","`StrippedUrl` serves as a key in request extensions. It contains non-matched part of url if\nroute was mounted",null,null],[3,"NoRoute","","`NoRoute` is error type returned to Iron if no route was matched in Router",null,null],[11,"new","","Construct new `MethodPicker`.",0,{"inputs":[{"name":"methodpicker"}],"output":{"name":"methodpicker"}}],[11,"add","","Adds new handler to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"method"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"get","","Adds new handler for GET http method to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"post","","Adds new handler for POST http method to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"put","","Adds new handler for PUT http method to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"delete","","Adds new handler for DELETE http method to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"head","","Adds new handler for HEAD http method to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"patch","","Adds new handler for PATCH http method to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"options","","Adds new handler for OPTIONS http method to picker",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"default","","Adds default handler to picker which will be used if no method matched",0,{"inputs":[{"name":"methodpicker"},{"name":"h"}],"output":{"name":"methodpicker"}}],[11,"handle","","",0,{"inputs":[{"name":"methodpicker"},{"name":"request"}],"output":{"name":"ironresult"}}],[11,"fmt","","",1,{"inputs":[{"name":"nomethod"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"nomethod"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",1,{"inputs":[{"name":"nomethod"}],"output":{"name":"str"}}],[11,"new","","Construct new `Router`.",2,{"inputs":[{"name":"router"}],"output":{"name":"router"}}],[11,"add_route","","Add a new route to a `Router`, matching given pattern.",2,{"inputs":[{"name":"router"},{"name":"s"},{"name":"h"},{"name":"bool"}],"output":{"name":"router"}}],[11,"handle","","",2,{"inputs":[{"name":"router"},{"name":"request"}],"output":{"name":"ironresult"}}],[11,"clone","","",3,{"inputs":[{"name":"strippedurl"}],"output":{"name":"strippedurl"}}],[11,"fmt","","",4,{"inputs":[{"name":"noroute"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",4,{"inputs":[{"name":"noroute"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",4,{"inputs":[{"name":"noroute"}],"output":{"name":"str"}}],[6,"Params","","`Params` is stored in request extensions and gives access to captured dynamic parameters",null,null]],"paths":[[3,"MethodPicker"],[3,"NoMethod"],[3,"Router"],[3,"StrippedUrl"],[3,"NoRoute"]]};
initSearch(searchIndex);
