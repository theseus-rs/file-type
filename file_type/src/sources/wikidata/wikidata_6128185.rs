use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6128185: FileFormat = FileFormat {
    id: 6_128_185,
    source_type: SourceType::Wikidata,
    name: "SigmaTel Motion Video",
    extensions: &["smv"],
    media_types: &["video/x-smv"],
    signatures: &[],
    related_formats: &[],
};
