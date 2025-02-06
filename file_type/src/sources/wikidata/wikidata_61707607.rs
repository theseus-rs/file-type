use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61707607: FileFormat = FileFormat {
    id: 61_707_607,
    source_type: SourceType::Wikidata,
    name: "Microsoft Outlook Email Message",
    extensions: &["msg", "oft"],
    media_types: &["application/vnd.ms-outlook"],
    signatures: &[],
    related_formats: &[],
};
