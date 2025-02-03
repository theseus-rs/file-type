use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29651319: FileFormat = FileFormat {
    id: 29_651_319,
    source_type: SourceType::Wikidata,
    name: "PixieScript",
    extensions: &["pxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
