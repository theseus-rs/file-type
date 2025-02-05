use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66208329: FileFormat = FileFormat {
    id: 66_208_329,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works communications script file",
    extensions: &["wcm"],
    media_types: &["application/vnd.ms-works"],
    signatures: &[],
    related_formats: &[],
};
