use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_89347372: FileFormat = FileFormat {
    id: 89_347_372,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Document 4",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[],
    related_formats: &[],
};
