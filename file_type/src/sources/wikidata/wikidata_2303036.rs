use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2303036: FileFormat = FileFormat {
    id: 2_303_036,
    source_type: SourceType::Wikidata,
    name: "WWF, unprintable PDF",
    extensions: &["wwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
