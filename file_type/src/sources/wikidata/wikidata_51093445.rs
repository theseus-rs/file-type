use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51093445: FileFormat = FileFormat {
    id: 51_093_445,
    source_type: SourceType::Wikidata,
    name: "Microsoft Outlook Address Book",
    extensions: &["olk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
