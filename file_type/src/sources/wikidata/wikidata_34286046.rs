use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34286046: FileFormat = FileFormat {
    id: 34_286_046,
    source_type: SourceType::Wikidata,
    name: "Pixie script",
    extensions: &["pxi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
