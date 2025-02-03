use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47520795: FileFormat = FileFormat {
    id: 47_520_795,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version 11",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
