use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2303036: FileFormat = FileFormat {
    id: 2_303_036,
    puid: "wikidata/2303036",
    name: "WWF, unprintable PDF",
    extensions: &["wwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
