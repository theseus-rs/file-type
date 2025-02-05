use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63061514: FileFormat = FileFormat {
    id: 63_061_514,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word Document, version 97-2003",
    extensions: &["doc", "wbk"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
