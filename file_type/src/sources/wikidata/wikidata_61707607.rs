use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61707607: FileFormat = FileFormat {
    id: 61_707_607,
    puid: "wikidata/61707607",
    name: "Microsoft Outlook Email Message",
    extensions: &["msg", "oft"],
    media_types: &["application/vnd.ms-outlook", "application/vnd.ms-outlook"],
    internal_signatures: &[],
    related_formats: &[],
};
