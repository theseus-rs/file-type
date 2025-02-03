use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_92744208: FileFormat = FileFormat {
    id: 92_744_208,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 2019",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[],
    related_formats: &[],
};
