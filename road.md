Road : 

/folder  :
    get :
        // cas client -> ses folder
        cas conseillier -> tout les folder
    post : 
        {
            address : {
                city : "",
                street : "",
                zipcode : ""
            },
            surface : 0.0,
            folder_num : "",
            // id_customer : 11111, # Cas conseillier Option<u32>
            tax_income : 135454.15,
            situation : ""
        }

/folder/{id}

/projet/{folder_id}

























# prototype in case
/folder/customer/{id}
    cas conseillier -> projet du client
