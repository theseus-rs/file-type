use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_89344774: FileFormat = FileFormat {
    id: 89_344_774,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Document 3.1",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
