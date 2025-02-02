use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27895555: FileFormat = FileFormat {
    id: 27_895_555,
    source_type: SourceType::Wikidata,
    name: "ADX, version 3",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
