use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_92204260: FileFormat = FileFormat {
    id: 92_204_260,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 2018",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
