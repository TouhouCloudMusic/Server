table "user" {
  schema = schema.public

  column "id" {
    null = false
    type = int
		identity {
			generated = BY_DEFAULT
		}
  }
  primary_key {
    columns = [column.id]
  }

  column "name" {
    null = false
    type = text
  }

  column "password" {
    null = false
    type = text
  }

	column "avatar_id" {
		type = int
	}
	foreign_key "avatar_id" {
		columns = [ column.avatar_id ]
		ref_columns = [ table.image.column.id ]
		on_update = CASCADE
		on_delete = SET_NULL
	}

	index "idx_name" {
		columns = [column.name]
		unique = true
	}
}
