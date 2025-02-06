use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_82025103: FileFormat = FileFormat {
    id: 82_025_103,
    source_type: SourceType::Wikidata,
    name: "Microsoft Reader eBook annotations",
    extensions: &["ebo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
