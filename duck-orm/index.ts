import { P, S, T } from "./orm";

export type User = S.Table<{
	id: P.Default<P.PK<P.Unique<T.UUID>>, "uuidv4">;
	name: P.PK<T.VarChar<255>>;
	bio: T.Text;
	role: T.Enum<"admin" | "user">;
	platform: T.Enum<"android" | "ios">;
	versions: T.VarChar<255> | P.Default<T.Integer, 0>;
	updated_at: P.Default<P.Time<"timestamp">, "now">;
	created_at: P.Default<P.Time<"timestamp">, "now">;
}>;

export type Course = S.Table<{
	id: P.PK<P.UUID>;
	name: P.PK<T.VarChar<255>>;
	progress: P.GreaterThan<T.Integer, 0> & P.LessThan<T.Integer, 100>;
	other_information: T.JSONB;
	user_id: P.OnUpdate<
		P.OnDelete<P.PK<S.Table<User>["id"]>, "CASCADE">,
		"RESTRICT"
	>;
}>;
