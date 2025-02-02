use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51093445: FileFormat = FileFormat {
    id: 51_093_445,
    source_type: SourceType::Wikidata,
    name: "Microsoft Outlook Address Book",
    extensions: &["olk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
