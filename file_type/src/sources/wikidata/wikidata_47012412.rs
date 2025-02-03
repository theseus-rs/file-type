use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47012412: FileFormat = FileFormat {
    id: 47_012_412,
    source_type: SourceType::Wikidata,
    name: "FoxPro Memo file format",
    extensions: &["fpt", "frt", "pjt", "vct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
