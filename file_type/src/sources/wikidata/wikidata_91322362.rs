use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_91322362: FileFormat = FileFormat {
    id: 91_322_362,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 2017",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
