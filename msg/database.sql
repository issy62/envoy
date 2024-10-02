-- Copyright 2024 Israel Bermudez Sepulveda
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
--     http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.

PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS msg_log (
    msg_id INTEGER PRIMARY KEY,
    msg_from TEXT NOT NULL,
    msg_to TEXT NOT NULL,
    msg_body TEXT NOT NULL,
    msg_operator TEXT NOT NULL,
    msg_timestamp TEXT NOT NULL,
    msg_resp TEXT NOT NULL
  );
COMMIT;

