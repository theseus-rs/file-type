use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205968: FileFormat = FileFormat {
    id: 28_205_968,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Blue Channel",
    extensions: &["imb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
