use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_89897874: FileFormat = FileFormat {
    id: 89_897_874,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 8",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[],
    related_formats: &[],
};
