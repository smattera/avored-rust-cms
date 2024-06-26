import {Link} from "react-router-dom"
import _ from 'lodash'
import {usePageTable} from "./hooks/usePageTable"
import { useTranslation } from "react-i18next"

function PageTable() {
    const page_api_table_response = usePageTable()
    const pages = _.get(page_api_table_response, 'data.data.data', [])
    const [t] = useTranslation("global")

    const getFormattedDate = ((date) => {
        const date_obj = new Date(date);

        return `${date_obj.getFullYear()}-${("0" + (date_obj.getMonth() + 1)).slice("-2")}-${("0" + date_obj.getDate()).slice("-2")}`;
    })



    return (
      <div className="flex-1 bg-white">
        <div className="px-5 ml-64">
          <div className="flex items-center">
            <div className="p-5 text-2xl font-semibold text-primary-500">
              {t("pages.pages")}
            </div>
            <Link
              className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              to="/admin/page-create"
            >
              {t("common.create")}
            </Link>
          </div>

          <div className="overflow-x-auto">
            <table className="min-w-full bg-white shadow-md rounded">
              <thead>
                <tr className="bg-gray-700 text-white">
                  <th className="py-3 px-4 rounded-l font-semibold text-left">
                    {t("common.id")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.name")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.identifier")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.created_at")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.updated_at")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.created_by")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.updated_by")}
                  </th>
                  <th className="py-3 px-4 rounded-r font-semibol text-left">
                    {t("common.action")}
                  </th>
                </tr>
              </thead>
              <tbody className="">
                {pages.map((page) => {
                  return (
                    <tr key={page.id} className="border-b">
                      <td className="py-3 px-4">{page.id}</td>
                      <td className="py-3 px-4">{page.name}</td>
                      <td className="py-3 px-4">{page.identifier}</td>
                      <td className="py-3 px-4">
                        {getFormattedDate(page.created_at)}
                      </td>
                      <td className="py-3 px-4">
                        {getFormattedDate(page.updated_at)}
                      </td>
                      <td className="py-3 px-4">{page.created_by}</td>
                      <td className="py-3 px-4">{page.updated_by}</td>
                      <td className="py-3 px-4">
                        <Link
                          className="font-medium text-primary-600 hover:text-primary-800"
                          to={`/admin/page-edit/${page.id}`}
                        >
                          {t("common.edit")}
                        </Link>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    );
}

export default PageTable