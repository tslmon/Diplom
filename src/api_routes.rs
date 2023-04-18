use actix_web::{web, HttpResponse};
use api::common::{
    job::jobs::JobApi,
    pool::{members::MemberApi, pools::PoolApi, candidates::{CandidateApi, CandidateDuplicateApi, CandidateMoveApi}},
};
use api::{
    actions::job::ManagementTrait,
    common::job::stages::{StagesApi, StagesStatusApi},
};
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/recruit/v1")
            // Health Check
            .service(web::scope("/health").route("", web::get().to(HttpResponse::Ok)))
            .service(
                web::scope("/users")
                    .route("", web::get().to(UserApi::get_collection))
                    .route("", web::post().to(UserApi::create_item))
                    .route("/{user_id}", web::get().to(UserApi::get_item))
                    .route("/{user_id}", web::put().to(UserApi::update_item))
                    .route("/{user_id}", web::delete().to(UserApi::delete_item))

                    
                    .route("", web::get().to(ProductApi::get_collection))
                    .route("", web::post().to(ProductApi::create_item))
                    .route("/{product_id}", web::get().to(ProductApi::get_item))
                    .route("/{product_id}", web::put().to(ProductApi::update_item))
                    .route("/{product_id}", web::delete().to(ProductApi::delete_item))


                    .route("", web::get().to(CategoryApi::get_collection))
                    .route("", web::post().to(CategoryApi::create_item))
                    .route("/{category_id}", web::get().to(CategoryApi::get_item))
                    .route("/{category_id}", web::put().to(CategoryApi::update_item))
                    .route("/{category_id}", web::delete().to(CategoryApi::delete_item))
                  
                    .route(
                        "/{job_id}/stages/{stage_id}",
                        web::get().to(StagesApi::get_item),
                    )
                    .route(
                        "/{job_id}/stages/{stage_id}",
                        web::put().to(StagesApi::update_item),
                    )
                    .route(
                        "/{job_id}/status",
                        web::post().to(StagesStatusApi::update_item),
                    )
                    .route(
                        "/{job_id}/stages/{stage_id}",
                        web::delete().to(StagesApi::delete_item),
                    )
                    // .route(
                    //     "/{job_id}/teachers",
                    //     web::get().to(TeacherApi::get_collection),
                    // )
                    .route(
                        "/{pool_id}/candidates",
                        web::get().to(CandidateApi::get_collection),
                    )
                    .route("/{pool_id}/candidates", web::post().to(CandidateApi::create_item))
                    .route("/{pool_id}/candidates/{candidate_id}", web::get().to(CandidateApi::get_item))
                    .route("/{pool_id}/candidates/{candidate_id}", web::put().to(CandidateApi::update_item))
                    .route( "/{pool_id}/candidates/{candidate_id}",web::patch().to(CandidateApi::update_item), )
                    .route("/{pool_id}/candidates/{candidate_id}", web::delete().to(CandidateApi::delete_item))
                    .route("/{pool_id}/candidates/{candidate_id}/move", web::post().to(CandidateMoveApi::update_item))
                    .route("/{pool_id}/candidates/{candidate_id}/duplicate", web::post().to(CandidateDuplicateApi::update_item))

                    .route(
                        "/{pool_id}/members",
                        web::get().to(MemberApi::get_collection),
                    )
                    .route("/{pool_id}/members", web::post().to(MemberApi::create_item))
                    .route(
                        "/{pool_id}/members/{user_id}",
                        web::get().to(MemberApi::get_item),
                    )
                    .route(
                        "/{pool_id}/members",
                        web::delete().to(MemberApi::delete_collection),
                    )
                    .route(
                        "/{pool_id}/members/{user_id}",
                        web::delete().to(MemberApi::delete_item),
                    ), 
                    // .route(
                       //     "/{job_id}/teachers",
                       //     web::post().to(TeacherApi::create_item),
                       // )
                       
                       // .route(
                       //     "/{job_id}/teachers/{teacher_id}",
                       //     web::get().to(TeacherApi::get_item),
                       // )
                       // .route(
                       //     "/{job_id}/teachers/{teacher_id}",
                       //     web::delete().to(TeacherApi::delete_item),
                       // )
                       // .route(
                       //     "/{job_id}/teachers",
                       //     web::delete().to(TeacherApi::delete_collection),
                       // )
                       // .route(
                       //     "{job_id}/invitations",
                       //     web::post().to(InvitationApi::create_item),
                       // )
                       // .route(
                       //     "/{job_id}/invitations/{invitation_id}",
                       //     web::get().to(InvitationApi::get_item),
                       // )
                       // .route(
                       //     "/{job_id}/invitations/{invitation_id}",
                       //     web::delete().to(InvitationApi::delete_item),
                       // )
                       // .route(
                       //     "/{job_id}/invitations/{invitation_id}/accept",
                       //     web::post().to(ClassroomAcceptApi::update_item),
                       // )
                       // .route(
                       //     "/{job_id}/invitations",
                       //     web::delete().to(InvitationApi::delete_collection),
                       // )
                       // .route(
                       //     "{job_id}/students",
                       //     web::get().to(StudentApi::get_collection),
                       // )
                       // .route(
                       //     "{job_id}/students",
                       //     web::post().to(StudentApi::create_item),
                       // )
                       // .route(
                       //     "/{job_id}/students/{student_id}",
                       //     web::get().to(StudentApi::get_item),
                       // )
                       // .route(
                       //     "/{job_id}/students/{student_id}",
                       //     web::delete().to(StudentApi::delete_item),
                       // )
                       // .route(
                       //     "/{job_id}/students",
                       //     web::delete().to(StudentApi::delete_collection),
                       // )
                       // .route(
                       //     "{job_id}/tools",
                       //     web::get().to(ToolApi::get_collection),
                       // )
                       // .route("{job_id}/tools", web::post().to(ToolApi::create_item))
                       // .route(
                       //     "/{job_id}/tools/{tool_id}",
                       //     web::get().to(ToolApi::get_item),
                       // )
                       // .route(
                       //     "/{job_id}/tools/{tool_id}",
                       //     web::put().to(ToolApi::update_item),
                       // )
                       // .route(
                       //     "/{job_id}/tools/{tool_id}",
                       //     web::delete().to(ToolApi::delete_item),
            ),
    );
}
