use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47012412: FileFormat = FileFormat {
    id: 47_012_412,
    source_type: SourceType::Wikidata,
    name: "FoxPro Memo file format",
    extensions: &["fpt", "frt", "pjt", "vct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
