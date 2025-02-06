use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117313902: FileFormat = FileFormat {
    id: 117_313_902,
    source_type: SourceType::Wikidata,
    name: "Clear Text CGM",
    extensions: &["ctm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
