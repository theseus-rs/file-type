use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66208329: FileFormat = FileFormat {
    id: 66_208_329,
    puid: "wikidata/66208329",
    name: "Microsoft Works communications script file",
    extensions: &["wcm"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
