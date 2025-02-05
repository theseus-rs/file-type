use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205725: FileFormat = FileFormat {
    id: 28_205_725,
    source_type: SourceType::Wikidata,
    name: "Async Professional Fax",
    extensions: &["apf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
