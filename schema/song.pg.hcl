table "song" {
	schema = schema.public

	column "id" {
		type = int
		identity {
			generated = BY_DEFAULT
		}
	}
	primary_key {
		columns = [ column.id ]
	}

	column "entity_id" {
		type = int
	}

	column "status" {
		type = enum.EntityStatus
	}

	column "title" {
		type = text
	}

	column "created_at" {
		type = timestamptz
		default = sql("now()")
	}

	column "updated_at" {
		type = timestamptz
		default = sql("now()")
	}
}

table "song_localized_title" {
	schema = schema.public

	column "id" {
		type = int
		identity {
			generated = BY_DEFAULT
		}
	}

	column "song_id" {
		type = int
	}
	foreign_key "song_id" {
		columns = [ column.song_id ]
		ref_columns = [ table.song.column.id ]
		on_update = CASCADE
		on_delete = CASCADE
	}

	column "language_id" {
		type = int
	}
	foreign_key "language_id" {
		columns = [ column.language_id ]
		ref_columns = [ table.language.column.id ]
		on_update = CASCADE
		on_delete = SET_NULL
	}

	column "title" {
		type = text
	}

}

table "song_credit" {
	schema = schema.public

	column "id" {
		type = int
		identity {
			generated = BY_DEFAULT
		}
	}
	primary_key {
		columns = [ column.id ]
	}

	column "artist_id" {
		type = int
	}

	column "song_id" {
		type = int
	}
	foreign_key "song_id" {
		columns = [ column.song_id ]
		ref_columns = [ table.song.column.id ]
		on_update = CASCADE
		on_delete = CASCADE
	}

	column "role_id" {
		type = int
	}
	foreign_key "role_id" {
		columns = [ column.role_id ]
		ref_columns = [ table.credit_role.column.id ]
		on_update = CASCADE
		on_delete = SET_NULL
	}
}


table "song_history" {
	schema = schema.public

	column "id" {
		type = int
		identity {
			generated = BY_DEFAULT
		}
	}

	column "prev_id" {
		type = int
	}
	foreign_key "prev_id" {
		columns = [ column.prev_id ]
		ref_columns = [ table.song.column.id ]
		on_update = CASCADE
		on_delete = CASCADE
	}

	column "next_id" {
		type = int
	}
	foreign_key "next_id" {
		columns = [ column.next_id ]
		ref_columns = [ table.song.column.id ]
		on_update = CASCADE
		on_delete = CASCADE
	}
}