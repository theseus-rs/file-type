use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_89777428: FileFormat = FileFormat {
    id: 89_777_428,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 7",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
