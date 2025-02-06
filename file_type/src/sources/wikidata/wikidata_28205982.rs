use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205982: FileFormat = FileFormat {
    id: 28_205_982,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Q Color Channel",
    extensions: &["imq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
