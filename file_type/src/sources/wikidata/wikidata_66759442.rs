use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66759442: FileFormat = FileFormat {
    id: 66_759_442,
    puid: "wikidata/66759442",
    name: "Microsoft Access Database Templates",
    extensions: &["accdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
