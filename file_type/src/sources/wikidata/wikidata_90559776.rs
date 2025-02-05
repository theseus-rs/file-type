use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_90559776: FileFormat = FileFormat {
    id: 90_559_776,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 9.1",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
