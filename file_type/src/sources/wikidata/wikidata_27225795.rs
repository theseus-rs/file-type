use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27225795: FileFormat = FileFormat {
    id: 27_225_795,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Graphics, version 1.0",
    extensions: &["odg"],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    signatures: &[],
    related_formats: &[],
};
