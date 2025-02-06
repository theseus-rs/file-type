use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_90801872: FileFormat = FileFormat {
    id: 90_801_872,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 2015",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
